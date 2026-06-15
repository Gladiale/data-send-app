import { SocketAddrProvider } from "@/context/socket-addr-context";

export default function TauriLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return <SocketAddrProvider>{children}</SocketAddrProvider>;
}
