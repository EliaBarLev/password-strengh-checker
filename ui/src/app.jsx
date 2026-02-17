import React, { useState } from "react";
import PasswordInput from "./components/PasswordInput.jsx";
import StrengthMeter from "./components/StrengthMeter.jsx";
import { scorePassword } from "./api/api.js";

function App() {
  const [password, setPassword] = useState("");
  const [result, setResult] = useState(null);
  const [loading, setLoading] = useState(false);

  const handleChange = async (pw) => {
    setPassword(pw);
    if (pw.length === 0) {
      setResult(null);
      return;
    }

    setLoading(true);
    try {
      const res = await scorePassword(pw);
      setResult(res.data);
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="app-container">
      <h1>Password Strength Checker 🔐</h1>
      <PasswordInput value={password} onChange={handleChange} />
      {loading && <p>Analyzing...</p>}
      {result && <StrengthMeter result={result} />}
    </div>
  );
}

export default App;
