import React from "react";
import { graphql, useStaticQuery } from "gatsby";
import Layout from "../components/layout";
import BlogList from "../components/blog/list/index";
import "../assets/css/main.css";

const BlogPage = () => {
  const data = useStaticQuery(query);
  return (
    <Layout seo={data.strapiHomepage.seo}>
      <div className="uk-section">
        <div className="uk-container uk-container-large">
          <BlogList articles={data.allStrapiArticle.edges} />
        </div>
      </div>
    </Layout>
  );
};

const query = graphql`
  query {
    strapiHomepage {
      hero {
        title
      }
      seo {
        metaTitle
        metaDescription
        shareImage {
          localFile {
            publicURL
          }
        }
      }
    }
    allStrapiArticle {
      edges {
        node {
          strapiId
          slug
          title
          updated_at(formatString: "YYYY-MM-DD HH:mm:ss")
          category {
            name
          }
          image {
            localFile {
              childImageSharp {
                gatsbyImageData(width: 800, height: 500)
              }
            }
          }
          author {
            name
            picture {
              localFile {
                childImageSharp {
                  gatsbyImageData(width: 30, height: 30)
                }
              }
            }
          }
        }
      }
    }
  }
`;

export default BlogPage;
