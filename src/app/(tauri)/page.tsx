"use client";
import IpQrCode from "@/components/ip-qr-code";

export default function TauriHome() {
  return (
    <main className="min-w-screen min-h-screen bg-[url(/images/bg/bg.jpg)] bg-cover bg-center bg-no-repeat">
      <IpQrCode />
    </main>
  );
}
