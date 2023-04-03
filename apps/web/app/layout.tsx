import { Analytics } from "@vercel/analytics/react";
import type { FC, ReactNode } from "react";
export { metadata } from './metadata';

import "./globals.css";

type LayoutProps = {
  children: ReactNode;
};

const Layout: FC<LayoutProps> = ({ children }) => {
  return (
    <html lang={"en"} data-theme={"dracula"}>
      <body>
        <main>{children}</main>
        <Analytics />
      </body>
    </html>
  );
};

export default Layout;
