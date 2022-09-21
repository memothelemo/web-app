import { Typography, Card } from "antd";
import React from "react";

const { Title } = Typography;

export default function UnavailableSubmissions() {
  return (
    <div className="Center">
      <Card className="Center" style={{ width: 500, height: 200 }}>
        <Title style={{ textAlign: "center" }} level={2}>
          Submit Letter
        </Title>
        <Typography>
          {
            "We're sad to report that we're not accepting any new submissions anymore."
          }
          <br />
          Thank you for visiting this site! I hope you will submit a letter in
          time next time!
        </Typography>
      </Card>
    </div>
  );
}
