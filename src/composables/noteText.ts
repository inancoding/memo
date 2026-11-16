// 便签正文是富文本 HTML，展示与搜索都需要纯文本，统一放在这里，避免各处各写一份。

// 卡片预览用：保留换行语义（<br> 与块级元素结束都算换行）
export function htmlToText(html: string): string {
  if (!html) return ''
  const el = document.createElement('div')
  el.innerHTML = html
  el.querySelectorAll('br').forEach(br => br.replaceWith(document.createTextNode('\n')))
  el.querySelectorAll('div, p, li').forEach(node => node.appendChild(document.createTextNode('\n')))
  return (el.textContent || '')
    .replace(/\u200B/g, '')
    .replace(/[ \t]+\n/g, '\n')
    .replace(/\n{2,}/g, '\n')
    .trim()
}

// 搜索用：不解析 DOM（列表过滤是热路径），先丢掉 <img> 整段，否则内嵌的 base64
// data URI 会形成长串误匹配；标签整体剥离后按渲染后的文字匹配。
export function searchableText(title: string, content: string): string {
  const body = content
    .replace(/<img[^>]*>/gi, ' ')
    .replace(/<[^>]*>/g, ' ')
    .replace(/&nbsp;/gi, ' ')
    .replace(/&amp;/gi, '&')
    .replace(/&lt;/gi, '<')
    .replace(/&gt;/gi, '>')
    .replace(/&quot;/gi, '"')
    .replace(/\u200B/g, '')
  return `${title} ${body}`.toLowerCase()
}
