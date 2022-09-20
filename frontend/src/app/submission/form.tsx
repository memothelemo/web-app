import { Button, Card, Checkbox, Input, Typography } from "antd";
import React from "react";
import "../main.css";

export interface PostLetterErrorInfo {
  message: string;
  inputMessageRed: boolean;
  authorRed: boolean;
}

interface Props {
  authorInput: string;
  messageInput: string;
  secretInput: boolean;

  error: {
    message: string;
    inputMessageRed: boolean;
    authorRed: boolean;
  };

  // eslint-disable-next-line no-unused-vars
  onAuthorChanged: (text: string) => void;
  // eslint-disable-next-line no-unused-vars
  onMessageChanged: (text: string) => void;
  // eslint-disable-next-line no-unused-vars
  onSecretChanged: (text: boolean) => void;

  onSubmit: () => void;

  disabled: boolean;
}

const { TextArea } = Input;

export default function SubmissionForm(props: Props) {
  return (
    <div className="Center">
      <Card className="Center" style={{ width: 550, height: 600 }}>
        <Typography.Title style={{ textAlign: "center" }} level={2}>
          Submit Letter
        </Typography.Title>
        <Typography>
          <b>Your goal is to send what do you want to say to the celebrant.</b>
        </Typography>
        <br />
        <Typography>
          <b>Note: </b>
          Your letter (if it is in public) will be published into the dashboard.
          <br />
          If you want to keep it in private, please click the checkbox says{" "}
          <br />
          <i>`Keep my letter secret`</i>
        </Typography>
        <br />
        <Input
          disabled={props.disabled}
          type="primary"
          status={props.error?.authorRed ? "error" : undefined}
          placeholder="Author"
          value={props.authorInput}
          onChange={e => {
            e.preventDefault();
            props.onAuthorChanged(e.target.value);
          }}
        />
        <TextArea
          disabled={props.disabled}
          style={{ marginTop: "10px", height: 200 }}
          status={props.error?.inputMessageRed ? "error" : undefined}
          placeholder="Message"
          maxLength={1000}
          autoSize={{ minRows: 2, maxRows: 8 }}
          showCount
          value={props.messageInput}
          onChange={e => {
            e.preventDefault();
            props.onMessageChanged(e.target.value);
          }}
        />
        <Checkbox
          disabled={props.disabled}
          checked={props.secretInput}
          style={{ marginTop: "5px" }}
          onChange={e => {
            e.preventDefault();
            props.onSecretChanged(e.target.checked);
          }}
        >
          Keep my letter secret
        </Checkbox>
        <Button
          disabled={props.disabled}
          type="primary"
          style={{
            marginTop: "10px",
            width: "100%",
          }}
          onClick={e => {
            e.preventDefault();
            props.onSubmit();
          }}
        >
          Submit
        </Button>
        {props.error?.message && <Typography>{props.error.message}</Typography>}
      </Card>
    </div>
  );
}
