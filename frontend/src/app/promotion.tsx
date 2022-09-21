import React from "react";
import { GoMarkGithub } from "react-icons/go";

export default function OSSPromotion() {
  return (
    <div
      style={{
        marginTop: "10px",
        textAlign: "center",
      }}
    >
      <b>The source code for this website is available on GitHub!</b>
      <br />
      <GoMarkGithub />{" "}
      <a
        target={"_blank"}
        rel={"noopener noreferrer"}
        href="https://github.com/memothelemo/web-app"
      >
        memothelemo/web-app
      </a>
    </div>
  );
}
