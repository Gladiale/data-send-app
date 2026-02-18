"use client";
import { useMemo, useRef, useState, useTransition } from "react";
import { BsUpload } from "react-icons/bs";
import { PiUploadThin } from "react-icons/pi";
import { RiDeleteBin2Line } from "react-icons/ri";
import { type UploadResType } from "@/types";
import Divider from "./divider";

/* 
  参照: https://zenn.dev/nbr41to/articles/39607375ba5aa8

  前提知識としてe.target.files (型: FileList)によって取得できるfilesは命名の通り複数です.
  そしてこのFileListは配列なように見えて配列ではなく,配列のメソッドは使用できません.
*/
const Upload = () => {
  const inputRef = useRef<HTMLInputElement>(null);
  const [isPending, startTransition] = useTransition();
  const [inputFiles, setInputFiles] = useState<FileList | null>(null);
  const [response, setResponse] = useState<UploadResType | null>(null);

  // 配列のメソッドを使えるようにFileList型をFile[]に変更
  const selectedFileArray: File[] = useMemo(() => {
    return inputFiles ? [...Array.from(inputFiles)] : [];
  }, [inputFiles]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!e.target.files) return;
    if (!inputRef.current?.files) return;

    // 前回レスポンスの状態をリセット
    setResponse(null);

    const newFileArray = [...selectedFileArray, ...Array.from(e.target.files)].filter(
      // .findIndex() 合致する最初の要素のインデックス
      // .filter() 配列から条件に一致する要素だけを抽出
      (file, index, self) => self.findIndex((f) => f.name === file.name) === index, // 「最初に見つかった位置」と「現在の位置」を比較して重複を削除
    );

    const dt = new DataTransfer();
    newFileArray.forEach((file) => dt.items.add(file));
    inputRef.current.files = dt.files; // input内のFileListを更新
    setInputFiles(dt.files); // Reactのstateを更新
  };

  const handleDelete = (index: number) => {
    if (!inputRef.current?.files) return;
    const dt = new DataTransfer();
    selectedFileArray.forEach((file, i) => i !== index && dt.items.add(file));
    inputRef.current.files = dt.files; // input内のFileListを更新
    setInputFiles(dt.files); // Reactのstateを更新
  };

  const handleUpload = async () => {
    if (!inputRef.current?.files) return;

    // 前回レスポンスの状態をリセット
    setResponse(null);

    const formData = new FormData();

    // 重要: 配列をそのまま渡すのではなく、ループで一つずつ append する
    selectedFileArray.forEach((file) => {
      formData.append("files", file);
    });

    try {
      const response = await fetch(`/axum-api/upload`, {
        method: "POST",
        body: formData,
      });

      if (!response.ok) {
        const err = await response.text();
        throw new Error(err);
      }

      const res = (await response.json()) as UploadResType;
      setResponse(res);
    } catch (error) {
      if (error instanceof Error) throw error;
      throw new Error("APIフェッチに失敗しました！");
    }
  };

  return (
    <div className="container h-screen content-center w-fit mx-auto relative">
      <div className="w-100 aspect-8/5 bg-[url(/images/bg/AliceInWonderland.png)] bg-cover bg-center bg-no-repeat absolute" />
      <div
        onClick={() => inputRef.current?.click()}
        className="w-100 aspect-8/5 bg-[#dfccf065] backdrop-blur-xs border border-dashed border-amber-400 content-center cursor-pointer"
      >
        <BsUpload size={30} color="#b7bf16" className="w-fit mx-auto" />
        <p className="text-[#acb418] text-md w-fit mx-auto mt-0.5">ファイルを選択</p>
        <input type="file" multiple onChange={handleChange} ref={inputRef} hidden />
      </div>

      {selectedFileArray.length > 0 && (
        <div className="w-full">
          <Divider className="mt-1" />

          <div className="space-y-0.5 my-1">
            {selectedFileArray.map((file, index) => {
              const target = response?.find(
                (item) => item.file_name === file.name && item.uploaded,
              );

              return (
                <div key={file.name} className="h-7 flex items-center justify-between">
                  <p className="w-80 truncate text-[#cb76ce] bg-[#d2e69b] px-0.5 text-sm h-full content-center text-center relative">
                    {file.name}
                    <span
                      className={`text-sm px-2 absolute left-0 ${target ? "" : "hidden"} ${target?.uploaded ? "text-green-500" : "text-red-500"}`}
                    >
                      {target?.uploaded ? "success" : "failed"}
                    </span>
                  </p>
                  <button
                    type="button"
                    onClick={() => handleDelete(index)}
                    className="h-full bg-[#b924a2] text-amber-200 flex-1 flex items-center justify-center gap-2 cursor-pointer hover:text-red-500 hover:bg-black transition-colors duration-300"
                  >
                    <RiDeleteBin2Line />
                    {target ? "クリア" : "削除"}
                  </button>
                </div>
              );
            })}
          </div>

          <Divider className="rotate-x-180 mb-1" />

          <button
            onClick={() =>
              startTransition(async () => {
                await handleUpload();
              })
            }
            type="button"
            className="active:border-none active:outline-none h-8 w-full flex items-center justify-center gap-2 bg-[#edcdf4d4] text-[#6e3573] hover:text-amber-200 hover:bg-[#c02b87] active:scale-95 transition duration-300 cursor-pointer text-[1rem]"
          >
            <PiUploadThin />
            {isPending ? "uploading ..." : "アップロード"}
          </button>
        </div>
      )}
    </div>
  );
};

export default Upload;
