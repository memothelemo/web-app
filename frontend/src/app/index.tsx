import React from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";

import "antd/dist/antd.css";

import DashboardPage from "./dashboard";
import LoginPage from "./login";
import SubmissionPage from "./submission";

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<SubmissionPage />} />
        <Route path="/login" element={<LoginPage />} />
        <Route path="/dashboard" element={<DashboardPage />} />
      </Routes>
    </BrowserRouter>
  );
}
