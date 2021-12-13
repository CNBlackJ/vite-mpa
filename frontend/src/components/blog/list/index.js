import React from "react";
import Card from "../card";

const List = ({ articles }) => {
  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6">
      <div className="flex flex-col items-center">
        {articles.map((article, i) => {
          return <Card article={article} />;
        })}
      </div>
    </div>
  );
};

export default List;
