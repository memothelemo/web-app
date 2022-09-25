import { Button, Card, Input, Typography } from "antd";
import React from "react";
import "../main.css";

export interface LoginFormError {
  why: string;
  password: boolean;
  username: boolean;
}

interface Props {
  username: string;
  password: string;

  error?: LoginFormError;

  // eslint-disable-next-line no-unused-vars
  onUsernameChanged: (text: string) => void;

  // eslint-disable-next-line no-unused-vars
  onPasswordChanged: (text: string) => void;

  onSubmit: () => void;
  disabled: boolean;
}

export default function LoginForm(props: Props) {
  return (
    <div className="Center">
      <Card className="Center" style={{ width: 300, height: 250 }}>
        <Typography.Title style={{ textAlign: "center" }} level={2}>
          Login
        </Typography.Title>
        <Input
          disabled={props.disabled}
          status={props.error?.username ? "error" : undefined}
          id="username"
          placeholder="Username"
          onChange={e => {
            e.preventDefault();
            props.onUsernameChanged(e.target.value);
          }}
          value={props.username}
        />
        <Input
          disabled={props.disabled}
          id="password"
          status={props.error?.password ? "error" : undefined}
          placeholder="Password"
          type="password"
          onChange={e => {
            e.preventDefault();
            props.onPasswordChanged(e.target.value);
          }}
          value={props.password}
        />
        <br />
        <Button
          disabled={props.disabled}
          type="primary"
          style={{ marginTop: "10px", width: "100%" }}
          onClick={e => {
            e.preventDefault();
            props.onSubmit();
          }}
        >
          Login
        </Button>
        {props.error?.why && <Typography>{props.error.why}</Typography>}
      </Card>
    </div>
  );
}
