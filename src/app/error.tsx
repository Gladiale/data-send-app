"use client";

export default function Error({ error }: { error: Error }) {
  return (
    <div className="text-red-500">
      <h2>エラーが発生しました！</h2>
      <p>{error.message}</p>
    </div>
  );
}
