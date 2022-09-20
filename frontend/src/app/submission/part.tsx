import axios from "axios";
import React, { useReducer, useState } from "react";
import { tripleCase } from "../../utils/tripleCase";
import SubmissionForm, { PostLetterErrorInfo } from "./form";

type DiagReducerActions =
  | {
      type: "AUTHOR_REQUIRED";
    }
  | { type: "MISSING_FIELDS" }
  | {
      type: "MESSAGE_REQUIRED";
    }
  | {
      type: "MESSAGE_TOO_SHORT";
    }
  | {
      type: "NONE";
    }
  | {
      type: "PUBLISH_FAILED";
      message?: string;
    };

const INITIAL_STATE: PostLetterErrorInfo = {
  message: "",
  authorRed: false,
  inputMessageRed: false,
};

const submissionReducer = (
  state: PostLetterErrorInfo,
  action: DiagReducerActions,
): PostLetterErrorInfo => {
  switch (action.type) {
    case "MISSING_FIELDS":
      return {
        message: "Both fields are required",
        inputMessageRed: true,
        authorRed: true,
      };
    case "AUTHOR_REQUIRED":
      return {
        message: "Author is required",
        inputMessageRed: false,
        authorRed: true,
      };
    case "MESSAGE_REQUIRED":
      return {
        message: "Message is required",
        inputMessageRed: true,
        authorRed: false,
      };
    case "MESSAGE_TOO_SHORT":
      return {
        message: "Message is short to send a letter!",
        inputMessageRed: true,
        authorRed: false,
      };
    case "PUBLISH_FAILED":
      return {
        inputMessageRed: false,
        authorRed: false,
        message:
          action.message ?? "Something wrong with your input, please try again",
      };
    case "NONE":
    default:
      return {
        ...INITIAL_STATE,
      };
  }
};

interface Props {
  onSubmitted: () => void;
}

export default function SubmissionPart(props: Props) {
  const [info, setInfo] = useState({
    author: "",
    message: "",
    secret: false,
  });
  const [diag, dispatchDiag] = useReducer(submissionReducer, INITIAL_STATE);
  const [disabled, setDisabled] = useState(false);

  async function submitLetterAPI() {
    try {
      const response = await axios.post("api/letters/post", { ...info });
      if (response.status !== 200) {
        dispatchDiag({
          type: "PUBLISH_FAILED",
          message:
            response.status === 429 ? "You're being ratelimited!" : undefined,
        });
      } else {
        props.onSubmitted();
      }
    } catch (reason) {
      console.error("[auth]", reason);
      dispatchDiag({ type: "PUBLISH_FAILED" });
    }
    setDisabled(false);
  }

  function submitLetterPrereqs() {
    const noMessage = !info.message;
    const noAuthor = !info.author;
    if (noMessage || noAuthor) {
      const action = tripleCase<DiagReducerActions["type"]>(
        noMessage,
        noAuthor,
        () => "MISSING_FIELDS",
        () => "AUTHOR_REQUIRED",
        () => "MESSAGE_REQUIRED",
        () => "PUBLISH_FAILED",
      );
      return dispatchDiag({ type: action });
    }
    if (info.message.length < 50) {
      return dispatchDiag({ type: "MESSAGE_TOO_SHORT" });
    }
    dispatchDiag({ type: "NONE" });
    setDisabled(true);
    submitLetterAPI();
  }

  return (
    <SubmissionForm
      authorInput={info.author}
      messageInput={info.message}
      secretInput={info.secret}
      error={diag}
      onAuthorChanged={author =>
        setInfo({ author: author, message: info.message, secret: info.secret })
      }
      onMessageChanged={message =>
        setInfo({ author: info.author, message: message, secret: info.secret })
      }
      onSecretChanged={secret =>
        setInfo({ author: info.author, message: info.message, secret: secret })
      }
      onSubmit={() => submitLetterPrereqs()}
      disabled={disabled}
    />
  );
}
