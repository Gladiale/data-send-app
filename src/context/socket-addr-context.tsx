"use client";
import { createContext, useContext, useState } from "react";

type SocketAddrContextType = {
  socketAddr: string | null;
  setSocketAddr: React.Dispatch<React.SetStateAction<string | null>>;
};

const SocketAddrContext = createContext({} as SocketAddrContextType);

const SocketAddrProvider = ({ children }: { children: React.ReactNode }) => {
  const [socketAddr, setSocketAddr] = useState<string | null>(null);

  return (
    <SocketAddrContext value={{ socketAddr, setSocketAddr }}>
      {children}
    </SocketAddrContext>
  );
};

const useSocketAddr = () => {
  return useContext(SocketAddrContext);
};

export { SocketAddrProvider, useSocketAddr };
