import React, { useState } from "react";
import SubmissionPart from "./part";
import "../main.css";
import { Card, Typography } from "antd";

function SubmissionPublished() {
  return (
    <div className="Center">
      <Card className="Center" style={{ width: 600, height: 300 }}>
        <Typography.Title style={{ textAlign: "center" }} level={2}>
          Congratulations!
        </Typography.Title>
        <Typography>
          You successfully sent a letter to a celebrant.
          <br />
          If you want to see all letter submissions (excluding secret ones),
          please head over to:
          <br />
          <a href={`${window.location.href}dashboard`}>
            {window.location.href}dashboard
          </a>
        </Typography>
      </Card>
    </div>
  );
}

export default function SubmissionPage() {
  const [submitted, setSubmitted] = useState(false);
  return submitted ? (
    <SubmissionPublished />
  ) : (
    <SubmissionPart onSubmitted={() => setSubmitted(true)} />
  );
}
