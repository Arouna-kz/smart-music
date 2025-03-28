import FooterOne from "@/Components/Footer/FooterOne";
import NavBar from "@/Components/Header/NavBar";
import "@/styles/globals.css";
import type { AppProps } from "next/app";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <NavBar />
      <Component {...pageProps} />
      <FooterOne/>
    </>
  );
}
