const Page = async () => {
  const { greet } = await import("greet");

  return (
    <h1 className={"text-primary"}>{greet("World")}</h1>
  );
};

export default Page;
