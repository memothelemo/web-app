import { Card, List, Typography } from "antd";
import axios from "axios";
import React, { useEffect, useState } from "react";
import { Letter } from "../types";
import { DateTime } from "luxon";
import { useMediaQuery } from "react-responsive";

const LetterCard: React.FC<Letter> = letter => {
  const date = DateTime.fromISO(letter.created_at);
  return (
    <Card>
      <Typography.Title style={{ fontSize: 20 }}>
        {letter.author}
      </Typography.Title>
      <Typography>{letter.message}</Typography>
      <Typography
        style={{ color: "rgb(127, 127, 127)" }}
      >{`Created on ${date.toLocaleString()} at ${date.toLocaleString({
        hour: "2-digit",
        minute: "2-digit",
      })}`}</Typography>
    </Card>
  );
};

export default function DashboardPage() {
  const [letters, setLetters] = useState<Letter[]>([]);
  const isMobile = useMediaQuery({ query: `(max-width: 760px)` });

  useEffect(() => {
    console.log("[DashboardPage] fetching all public letters");
    const c = axios.CancelToken.source();
    axios
      .get("api/letters/public", { cancelToken: c.token })
      .then(res => setLetters(res.data))
      .catch(err => {
        if (axios.isCancel(err)) {
          console.log("cancelled");
        } else {
          // TODO: handle error
        }
      });
  }, []);

  return (
    <div
      style={{
        margin: "30px",
        justifyContent: "center",
        display: "flex",
      }}
    >
      <List
        style={{
          width: isMobile ? 400 : 700,
        }}
        itemLayout="vertical"
        dataSource={letters}
        renderItem={v => <LetterCard key={v.id} {...v} />}
      />
    </div>
  );
}
