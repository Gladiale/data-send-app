import { AppCommand } from "@/libs/commands/app-command";
import { GiFullFolder } from "react-icons/gi";

const OpenFolders = () => {
  const handleUploadFolderClick = async () => {
    await AppCommand.openFolder("./upload");
  };

  return (
    <div className="h-20 border border-dashed border-purple-500 flex items-center justify-around relative">
      <div className="px-1 h-fit text-xs bg-[#ffffff65] text-[#967d11] backdrop-blur-md uppercase border border-dashed border-purple-500 absolute top-0 -translate-y-[50%] left-[3%]">
        folders
      </div>

      <button
        type="button"
        onClick={handleUploadFolderClick}
        className="h-[90%] aspect-square p-1.5 text-[#51324f] hover:text-[#f39fec] active:scale-95 flex flex-col items-center justify-center bg-amber-200 rounded-full transition-all duration-300 cursor-pointer"
      >
        <GiFullFolder className="text-5xl" />
        <span className="text-xs uppercase leading-2">upload</span>
      </button>
    </div>
  );
};

export default OpenFolders;
