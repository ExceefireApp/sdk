import { useState } from "react";

export default function Home() {
  const [size, setSize] = useState(0);
  const [leverage, setLeverage] = useState(1);

  const handleTrade = async () => {
    alert(`Opening position: ${size} with ${leverage}x`);
  };

  return (
    <div style={{ padding: 40 }}>
      <h1>🔥 Exceefire Trading</h1>

      <input
        type="number"
        placeholder="Size"
        onChange={(e) => setSize(Number(e.target.value))}
      />

      <input
        type="number"
        placeholder="Leverage"
        onChange={(e) => setLeverage(Number(e.target.value))}
      />

      <button onClick={handleTrade}>Open Position</button>
    </div>
  );
}
