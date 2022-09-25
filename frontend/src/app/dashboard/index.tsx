import { DownOutlined } from "@ant-design/icons";
import {
  Button,
  Card,
  Drawer,
  Dropdown,
  Form,
  Input,
  List,
  Menu,
  Skeleton,
  Space,
  Typography,
} from "antd";
import axios from "axios";
import React, { useEffect, useState } from "react";
import { DateTime } from "luxon";
import { useMediaQuery } from "react-responsive";
import { useCookies } from "react-cookie";

type DataType = {
  created_at?: string;
  author?: string;
  id?: string;
  message?: string;
  secret?: boolean;
  onReportAbuse?: () => void;
  loading: boolean;
};

const VIOLATIONS = {
  Spam: [1, "Spam"] as const,
  Abuse: [2, "Abuse to the website"] as const,
  BugReport: [3, "Bug report"] as const,
  TechnicalIssue: [4, "Technical issue"] as const,
  Profanity: [5, "Profanity"] as const,
  Bullying: [6, "Bullying to the celebrant"] as const,
  InapproriateContent: [7, "Inapproriate content"] as const,
  NSFW: [8, "NSFW (Not safe for work)"] as const,
  Scam: [9, "Scam/misleading content"] as const,
  Irrelevant: [10, "Irrelevant"] as const,
  Others: [11, "Unknown"] as const,
};
const VIOLATIONS_IN_MESSAGE = Object.values(VIOLATIONS).map(([, v]) => v);
const COUNT = 10;

interface AntdFieldData {
  name: string | number | (string | number)[];
  value?: any;
  touched?: boolean;
  validating?: boolean;
  errors?: string[];
}

interface ReportFormProps {
  // eslint-disable-next-line no-unused-vars
  onChange: (fields: AntdFieldData[]) => void;
  fields: AntdFieldData[];
}

const ReportAbuseForm: React.FC<ReportFormProps> = ({ onChange, field }) => {
  return (
    <Form layout="vertical">
      <Form.Item
        name="email"
        label="Email Address"
        rules={[{ required: true, message: "Email address is required!" }]}
      >
        <Input />
      </Form.Item>
    </Form>
  );
};

const LetterCard: React.FC<DataType> = letter => {
  let date: DateTime | undefined;
  if (letter.created_at) {
    date = DateTime.fromISO(letter.created_at);
  }
  return (
    <List.Item>
      <Skeleton title={false} loading={letter.loading} active>
        <Card>
          <Typography style={{ padding: "0px" }}>
            <h4 style={{ margin: "0px", padding: "0px" }}>{letter.author}</h4>
            {letter.secret ? (
              <p style={{ color: "#db4818", margin: "0px" }}> SECRET LETTER</p>
            ) : null}
          </Typography>
          <Typography>{letter.message}</Typography>
          <Typography
            style={{ marginTop: "10px", color: "rgb(127, 127, 127)" }}
          >
            Created in {date?.toLocaleString()} at{" "}
            {date?.toLocaleString({ hour: "2-digit", minute: "2-digit" })}`
          </Typography>
          <Typography.Link
            style={{ color: "#e22006" }}
            onClick={e => {
              e.preventDefault();
              letter.onReportAbuse?.();
            }}
            target="_blank"
          >
            <b>Report Abuse</b>
          </Typography.Link>
        </Card>
      </Skeleton>
    </List.Item>
  );
};

export default function DashboardPage() {
  const [initLoading, setInitLoading] = useState(true);
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<DataType[]>([]);
  const [list, setList] = useState<DataType[]>([]);
  const [offset, setOffset] = useState(0);
  const [empty, setEmpty] = useState(false);
  const [cookies] = useCookies(["token"]);
  const [isReportAbuseOpen, setReportAbuse] = useState(false);

  const isMobile = useMediaQuery({ query: `(max-width: 760px)` });
  let apiUrl = "api/letters";

  if (localStorage.getItem("viewer")) {
    apiUrl = "api/letters/all";
  }

  useEffect(() => {
    console.log("[DashboardPage] fetching all letters");
    const c = axios.CancelToken.source();
    axios
      .get(apiUrl, {
        withCredentials: true,
        headers: {
          authorization: cookies.token ? `Bearer ${cookies.token}` : "",
        },
        cancelToken: c.token,
      })
      .then(res => {
        if (res.data.length < 10) {
          setEmpty(true);
        }
        if (res.data.length != 0) {
          setOffset(res.data.length);
        }
        setInitLoading(false);
        setData(res.data);
        setList(res.data);
      })
      .catch(err => {
        if (axios.isCancel(err)) {
          console.log("cancelled");
        } else {
          // TODO: handle error
        }
      });
  }, []);

  const showReportPopup = () => {
    console.log("SHOTWING");
    setReportAbuse(true);
  };
  const closeReportPopup = () => setReportAbuse(false);

  const onLoadMore = () => {
    setLoading(true);
    setList(
      data.concat(
        [...new Array(COUNT)].map(() => ({
          loading: true,
        })),
      ),
    );
    const c = axios.CancelToken.source();
    axios
      .get(`${apiUrl}?offset=${offset}`, {
        withCredentials: true,
        headers: {
          authorization: cookies.token ? `Bearer ${cookies.token}` : "",
        },
        cancelToken: c.token,
      })
      .then(res => {
        const newData = data.concat(res.data);
        if (res.data.length < 10) {
          setEmpty(true);
        }
        if (res.data.length != 0) {
          setOffset(offset + res.data.length);
        }
        setData(newData);
        setList(newData);
        setLoading(false);
        window.dispatchEvent(new Event("resize"));
      })
      .catch(err => {
        if (axios.isCancel(err)) {
          console.log("cancelled");
        } else {
          // TODO: handle error
        }
      });
  };

  const loadMore =
    !initLoading && !empty && !loading ? (
      <div
        style={{
          textAlign: "center",
          marginTop: 12,
          height: 32,
          lineHeight: "32px",
        }}
      >
        <Button
          onClick={e => {
            e.preventDefault();
            onLoadMore();
          }}
        >
          Load More
        </Button>
      </div>
    ) : null;

  return (
    <>
      <Typography.Title
        style={{ marginTop: "20px", fontSize: "30px", textAlign: "center" }}
      >
        Public Letters
      </Typography.Title>
      <Drawer
        title="Report Abuse"
        width={300}
        onClose={closeReportPopup}
        open={isReportAbuseOpen}
      >
        <ReportAbuseForm />
      </Drawer>
      <div
        style={{
          justifyContent: "center",
          display: "flex",
        }}
      >
        <List
          style={{
            width: isMobile ? 400 : 700,
          }}
          loading={initLoading}
          loadMore={loadMore}
          itemLayout="vertical"
          dataSource={list.map((v, i) => [v, i] as const)}
          renderItem={([item, alt_id]) => (
            <LetterCard
              key={item.id ?? alt_id}
              onReportAbuse={showReportPopup}
              {...item}
            />
          )}
        />
      </div>
    </>
  );
}
