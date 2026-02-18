"use client";
import IpQrCode from "@/components/ip-qr-code";
import Upload from "@/components/upload";
import { useEffect, useState } from "react";

export default function Home() {
  const [tauriActive, setTauriActive] = useState<boolean>(false);
  const [mounted, setMounted] = useState<boolean>(false);

  useEffect(() => {
    setMounted(true);

    const hostname = window.location.hostname;
    if (hostname.includes("192.168")) {
      setTauriActive(false);
    } else {
      setTauriActive(true);
    }
  }, []);

  if (!mounted) {
    return (
      <main className="min-w-screen min-h-screen bg-[url(/images/bg/bg.jpg)] bg-cover bg-center bg-no-repeat"></main>
    );
  }

  return (
    <main className="min-w-screen min-h-screen bg-[url(/images/bg/bg.jpg)] bg-cover bg-center bg-no-repeat">
      {tauriActive && <IpQrCode />}
      {!tauriActive && <Upload />}
    </main>
  );
}
