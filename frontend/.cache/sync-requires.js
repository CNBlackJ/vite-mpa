
// prefer default export if available
const preferDefault = m => (m && m.default) || m


exports.components = {
  "component---cache-dev-404-page-js": preferDefault(require("/Users/zhangyesheng/workspace/vite-project/frontend/.cache/dev-404-page.js")),
  "component---src-pages-404-js": preferDefault(require("/Users/zhangyesheng/workspace/vite-project/frontend/src/pages/404.js")),
  "component---src-pages-article-strapi-article-slug-js": preferDefault(require("/Users/zhangyesheng/workspace/vite-project/frontend/src/pages/article/{StrapiArticle.slug}.js")),
  "component---src-pages-category-strapi-category-slug-js": preferDefault(require("/Users/zhangyesheng/workspace/vite-project/frontend/src/pages/category/{StrapiCategory.slug}.js")),
  "component---src-pages-index-js": preferDefault(require("/Users/zhangyesheng/workspace/vite-project/frontend/src/pages/index.js"))
}

