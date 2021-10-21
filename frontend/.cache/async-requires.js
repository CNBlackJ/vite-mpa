// prefer default export if available
const preferDefault = m => (m && m.default) || m

exports.components = {
  "component---cache-dev-404-page-js": () => import("./../../dev-404-page.js" /* webpackChunkName: "component---cache-dev-404-page-js" */),
  "component---src-pages-404-js": () => import("./../../../src/pages/404.js" /* webpackChunkName: "component---src-pages-404-js" */),
  "component---src-pages-article-strapi-article-slug-js": () => import("./../../../src/pages/article/{StrapiArticle.slug}.js" /* webpackChunkName: "component---src-pages-article-strapi-article-slug-js" */),
  "component---src-pages-category-strapi-category-slug-js": () => import("./../../../src/pages/category/{StrapiCategory.slug}.js" /* webpackChunkName: "component---src-pages-category-strapi-category-slug-js" */),
  "component---src-pages-index-js": () => import("./../../../src/pages/index.js" /* webpackChunkName: "component---src-pages-index-js" */)
}

