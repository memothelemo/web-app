import React, { useState, useEffect } from "react";
import SubmissionPart from "./part";
import "../main.css";
import { Card, Divider, Typography } from "antd";
import OSSPromotion from "../promotion";
import axios from "axios";
import UnavailableSubmissions from "./unavailable";

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
  const [available, setAvailable] = useState<null | boolean>(null);

  useEffect(() => {
    console.log("[DashboardPage] checking if submissions are available");
    const c = axios.CancelToken.source();
    axios
      .get("api/available", { cancelToken: c.token })
      .then(res => setAvailable(res.data.available))
      .catch(err => {
        if (axios.isCancel(err)) {
          console.log("cancelled");
        } else {
          // TODO: handle error
        }
      });
  }, []);

  return (
    <>
      {submitted ? (
        <SubmissionPublished />
      ) : available !== null ? (
        available ? (
          <SubmissionPart onSubmitted={() => setSubmitted(true)} />
        ) : (
          <UnavailableSubmissions />
        )
      ) : (
        <Typography>
          Please wait for a moment... If you think it is taking awhile to load,
          kindly refresh the page. (There might be problems to our internal
          server / API)
        </Typography>
      )}
      <Divider />
      <OSSPromotion />
    </>
  );
}
