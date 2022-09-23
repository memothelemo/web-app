import { Card, Divider, List, Typography } from "antd";
import axios from "axios";
import React, { useEffect, useReducer } from "react";
import { Letter } from "../types";
import { DateTime } from "luxon";
import { useMediaQuery } from "react-responsive";

interface State {
  loaded: boolean;
  letters: Letter[];
}

type Action = {
  type: "LETTERS_LOADED";
  letters: Letter[];
};

const reducer = (_state: State, action: Action): State => {
  switch (action.type) {
    case "LETTERS_LOADED":
      return {
        loaded: true,
        letters: [...action.letters],
      };
    default:
      return {
        loaded: false,
        letters: [],
      };
  }
};

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
  const [info, setInfo] = useReducer(reducer, {
    loaded: false,
    letters: [],
  });
  const isMobile = useMediaQuery({ query: `(max-width: 760px)` });

  useEffect(() => {
    console.log("[DashboardPage] fetching all public letters");
    const c = axios.CancelToken.source();
    axios
      .get("api/letters", { cancelToken: c.token })
      .then(res => setInfo({ type: "LETTERS_LOADED", letters: res.data }))
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
      <Typography.Title
        style={{ marginTop: "20px", fontSize: "30px", textAlign: "center" }}
      >
        Public Letters ({info.letters.length} in total)
      </Typography.Title>
      <Divider />
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
          dataSource={info.letters}
          renderItem={v => <LetterCard key={v.id} {...v} />}
        />
      </div>
    </>
  );
}
