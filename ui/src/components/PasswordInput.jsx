import React from "react";

export default function PasswordInput({ value, onChange }) {
  return (
    <input
      type="password"
      placeholder="Enter password"
      value={value}
      onChange={(e) => onChange(e.target.value)}
      className="password-input"
    />
  );
}
