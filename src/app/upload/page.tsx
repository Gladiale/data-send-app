"use client";
import { useMemo, useRef, useState } from "react";

/* 
  参照: https://zenn.dev/nbr41to/articles/39607375ba5aa8

  前提知識としてe.target.files (型: FileList)によって取得できるfilesは命名の通り複数です.
  そしてこのFileListは配列なように見えて配列ではなく,配列のメソッドは使用できません.
*/
const Upload = () => {
  const inputRef = useRef<HTMLInputElement>(null);
  const [inputFiles, setInputFiles] = useState<FileList | null>(null);

  // 配列のメソッドを使えるようにFileList型をFile[]に変更
  const selectedFileArray: File[] = useMemo(() => {
    return inputFiles ? [...Array.from(inputFiles)] : [];
  }, [inputFiles]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!e.target.files) return;
    if (!inputRef.current?.files) return;

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

  return (
    <main>
      <input type="file" multiple onChange={handleChange} ref={inputRef} />
      <div>
        {selectedFileArray.map((file, index) => (
          <div key={file.name}>
            <div>{file.name}</div>
            <button onClick={() => handleDelete(index)}>削除</button>
          </div>
        ))}
      </div>
    </main>
  );
};

export default Upload;
