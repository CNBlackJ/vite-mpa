import React from "react";

import { AuthingGuard } from "@authing/react-ui-components";
import "@authing/react-ui-components/lib/index.min.css";

const LoginPage = () => {
  return (
    <AuthingGuard
      appId="60dd3301930e7601756c2f23"
      onLogin={(userinfo) => {
        console.log(userinfo);
      }}
    />
  );
};

export default LoginPage;
