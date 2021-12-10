import React from "react";
import Card from "../card";

const List = ({ articles }) => {
  return (
    <div>
      {articles.map((article, i) => {
        return <Card article={article} />;
      })}
    </div>
  );
};

export default List;
