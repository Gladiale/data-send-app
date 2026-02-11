"use client";
import { useEffect, useState } from "react";
import { getSocketIp } from "@/libs/utils/get-local-ip";
import { useSocketAddr } from "@/context/socket-addr-context";
import { QRCodeCanvas } from "qrcode.react";

const IpQrCode = () => {
  const { socketAddr, setSocketAddr } = useSocketAddr();
  const [_, setError] = useState(null);

  useEffect(() => {
    const initSocketAddr = async () => {
      try {
        const ip = await getSocketIp();
        setSocketAddr(ip);
      } catch (error) {
        // 非同期関数（async）の内部で発生したエラー（throw error）は誰も受け取っていないので、単純の（throw error）はerror.tsxに反映されない
        // state更新時にエラーをセットし、次のレンダリングで throw する
        setError(() => {
          throw error;
        });
      }
    };

    initSocketAddr();
  }, []);

  return (
    <div>
      <p>ip: {socketAddr ? socketAddr : "取得中..."}</p>
      {/* 参照: https://zenn.dev/hayato94087/articles/fdb9fb357a22c3 */}
      {socketAddr && <QRCodeCanvas value={socketAddr} size={128} level={"L"} />}
    </div>
  );
};

export default IpQrCode;
