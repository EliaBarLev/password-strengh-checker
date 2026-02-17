import React from "react";

export default function StrengthMeter({ result }) {
  const { score, entropy_bits, bruteforce_time, leaked, reasons } = result;
  const color =
    score > 80 ? "green" : score > 50 ? "orange" : "red";

  return (
    <div className="strength-meter">
      <div className="bar-container">
        <div
          className="bar"
          style={{ width: `${score}%`, backgroundColor: color }}
        ></div>
      </div>
      <p>Score: {score}/100</p>
      <p>Entropy: {entropy_bits.toFixed(2)} bits</p>
      <p>Brute-force time: {bruteforce_time}</p>
      <p>Leaked: {leaked ? "Yes" : "No"}</p>
      {reasons.length > 0 && (
        <ul>
          {reasons.map((r, i) => (
            <li key={i}>{r}</li>
          ))}
        </ul>
      )}
    </div>
  );
}
