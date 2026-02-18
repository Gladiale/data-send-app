// レスポンスの型はバックエンド統一
type UploadResType = {
  file_name: string;
  uploaded: bool;
}[];

export { UploadResType };
