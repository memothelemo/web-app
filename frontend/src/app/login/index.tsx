import axios from "axios";
import React, { useReducer, useState } from "react";
import { useNavigate } from "react-router";
import { tripleCase } from "../../utils/tripleCase";
import LoginForm, { LoginFormError } from "./form";

type Action =
  | {
      type: "AUTHENTICATE";
      username: string;
      password: string;
    }
  | {
      type: "UNEXPECTED_ERROR";
    }
  | {
      type: "API_ERROR";
      message: string;
    };

const reducer = (
  _state: LoginFormError | undefined,
  action: Action,
): LoginFormError | undefined => {
  switch (action.type) {
    case "API_ERROR":
      return {
        why: action.message,
        username: false,
        password: false,
      };
    case "AUTHENTICATE": {
      const noUsername = !action.username;
      const noPassword = !action.password;
      if (noUsername || noPassword) {
        return {
          why: tripleCase(
            noUsername,
            noPassword,
            () => "Both fields are required",
            () => "Username is required",
            () => "Password is required",
            () => "unexpected error",
          ),
          username: noUsername,
          password: noPassword,
        };
      }
      return undefined;
    }
    case "UNEXPECTED_ERROR":
      return {
        why: "Unexpected error occurred. Please try to login again later",
        username: false,
        password: false,
      };
    default:
      return undefined;
  }
};

export default function LoginPage() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [disabled, setDisabled] = useState(false);
  const [diagnostic, dispatchDiagnostic] = useReducer(reducer, undefined);

  const navigate = useNavigate();

  return (
    <LoginForm
      username={username}
      password={password}
      disabled={disabled}
      error={diagnostic}
      onUsernameChanged={setUsername}
      onPasswordChanged={setPassword}
      onSubmit={() => {
        setDisabled(true);
        dispatchDiagnostic({
          type: "AUTHENTICATE",
          username,
          password,
        });

        const noUsername = !username;
        const noPassword = !password;
        if (noUsername || noPassword) return setDisabled(false);

        const instance = axios.create({
          withCredentials: true,
        });

        instance
          .post("/api/users/login", {
            username,
            password,
          })
          .then(async response => {
            if (response.status === 202) {
              localStorage.setItem("moderator", response.data.moderator);
              localStorage.setItem("viewer", response.data.viewer);
              if (response.data.moderator) {
                navigate("/reports");
              } else {
                navigate("/dashboard");
              }
              return;
            }
            let message =
              "Something wrong with the server, please try again later";
            if (response.headers?.authorization.endsWith("content-type")) {
              message = response.data.message;
            } else if (response.status === 429) {
              message = "You're being ratelimited";
            }
            dispatchDiagnostic({
              type: "API_ERROR",
              message,
            });
          })
          .catch(reason => {
            console.error(reason);
            dispatchDiagnostic({ type: "UNEXPECTED_ERROR" });
            setDisabled(false);
          });
      }}
    />
  );
}
