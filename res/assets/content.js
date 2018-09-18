import Contents from 'contents'

document.addEventListener("DOMContentLoaded", () => {
    const contents = Contents({
        articles: document.querySelectorAll('.content-body h2, .content-body h3, .content-body h4, .content-body h5, .content-body h6')
    })

    let holds = document.querySelectorAll('.hold')
    let handlers = []
    holds.forEach((el) => {
        let p = el.offsetParent
        let top = el.offsetTop
        while (p !== document.body) {
            top += p.offsetTop
            p = p.offsetParent
        }

        handlers.push({w: el.offsetWidth, top, el})
    })
    document.getElementById('toc').appendChild(contents.list())
    document.addEventListener('scroll', (e) => {
        let top = document.documentElement.scrollTop
        handlers.forEach((t) => {
            if (top >= t.top && !t.el.classList.contains('pin')) {
                t.el.classList.add('pin')
                t.el.style.width = `${t.w}px`
            } else if (top < t.top && t.el.classList.contains('pin')) {
                t.el.classList.remove('pin')
                t.el.style.width = ''
            }
        })
    })
})