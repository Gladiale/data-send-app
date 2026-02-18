"use client";
import { useEffect } from "react";
import { useSocketAddr } from "@/context/socket-addr-context";
import { QRCodeCanvas } from "qrcode.react";
import { AppCommand } from "@/libs/commands/app-command";
import { MdComputer } from "react-icons/md";
import { IoPhonePortraitOutline } from "react-icons/io5";
import { LiaExchangeAltSolid } from "react-icons/lia";
import OpenFolders from "./open_folders";

const IpQrCode = () => {
  const { socketAddr, setSocketAddr } = useSocketAddr();
  /*
  const [_, setError] = useState(null);

  useEffect(() => {
    const throwErrInUseEffect = async () => {
      try {
        ...
      } catch (error) {
        // 非同期関数（async）の内部で発生したエラー（throw error）は誰も受け取っていないので、単純の（throw error）はerror.tsxに反映されない
        // state更新時にエラーをセットし、次のレンダリングで throw する
        setError(() => {
          throw error;
        });
      }
    };
    throwErrInUseEffect();
  }, []);
  */

  useEffect(() => {
    AppCommand.getSocketAddr().then((addr) => setSocketAddr(addr));
  }, []);

  return (
    <div className="h-screen container content-center w-fit mx-auto space-y-7">
      <div className="shadow-md min-w-100 py-2 px-4 border-violet-300 bg-[#f3d8f57f] backdrop-blur-md rounded-xl text-[1.2rem] space-y-2">
        <div className="flex items-center justify-around text-4xl text-amber-200">
          <div className="py-1 px-2 bg-[#af6fb7] rounded-md">
            <MdComputer />
          </div>
          <LiaExchangeAltSolid color="#af65e0" />
          <div className="py-1 px-2 bg-[#af6fb7] rounded-md">
            <IoPhonePortraitOutline />
          </div>
        </div>

        {/* 参照: https://zenn.dev/hayato94087/articles/fdb9fb357a22c3 */}
        <div className="w-fit mx-auto min-h-80">
          {socketAddr && (
            <QRCodeCanvas
              value={socketAddr}
              size={320}
              level={"L"}
              bgColor={"#f9eafa"}
              fgColor={"#166d2ceb"}
              // imageSettings={{
              //   src: "/images/magic-circle.png",
              //   x: undefined,
              //   y: undefined,
              //   height: 30,
              //   width: 30,
              //   excavate: true,
              // }}
            />
          )}
        </div>

        <p className="leading-none w-fit mx-auto text-[#6e0767]">
          IP: {socketAddr ? socketAddr : "取得中..."}
        </p>
      </div>

      <OpenFolders />
    </div>
  );
};

export default IpQrCode;
