import React from "react";
import { GatsbyImage } from "gatsby-plugin-image";

const Card = ({ article }) => {
  return (
    <div className="mb-16 p-2 border rounded hover:border-opacity-100  border-opacity-0">
      <a
        href={`/article/${article.node.slug}`}
        className="text-black no-underline hover:no-underline"
      >
        <div className="w-96">
          <GatsbyImage
            image={article.node.image.localFile.childImageSharp.gatsbyImageData}
            alt={article.node.title}
            className="rounded"
          />
        </div>
        <div className="hover:text-sky-600">{article.node.title}</div>
        <div className="flex flex-row items-center justify-end">
          <div>{article.node.updated_at}</div>
        </div>
      </a>
    </div>
  );
};

export default Card;
