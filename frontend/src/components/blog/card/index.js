import React from "react";
import { Link } from "gatsby";
import { GatsbyImage } from "gatsby-plugin-image";

const Card = ({ article }) => {
  return (
    <Link to={`/article/${article.node.slug}`} className="uk-link-reset">
      <div>
        <GatsbyImage
          image={article.node.image.localFile.childImageSharp.gatsbyImageData}
          alt={`Hero image`}
        />
      </div>
      <div>{article.node.title}</div>
      <div>{article.node.author.name}</div>
    </Link>
  );
};

export default Card;
