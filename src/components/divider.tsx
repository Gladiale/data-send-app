type PropsType = {
  className?: string;
};

const Divider = ({ className }: PropsType) => {
  return (
    <div
      className={`bg-[url(/images/divider/img-divider.png)] w-full h-6 bg-contain bg-center bg-no-repeat ${className}`}
    />
  );
};

export default Divider;
