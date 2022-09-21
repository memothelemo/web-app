import { Divider, Typography } from "antd";
import React from "react";
import OSSPromotion from "./promotion";

export default function NotFoundPage() {
  return (
    <div style={{ margin: "20px" }}>
      <Typography.Title>Whoopsies!</Typography.Title>
      <Typography>{"We don't have kind of page for this! :)"}</Typography>
      <Divider />
      <OSSPromotion />
    </div>
  );
}
