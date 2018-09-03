import Contents from 'contents'

document.addEventListener("DOMContentLoaded", () => {
    const contents = Contents({
        articles: document.querySelectorAll('.content-body h2, .content-body h3, .content-body h4, .content-body h5, .content-body h6')
    })

    let hold = document.getElementsByClassName('hold')[0]
    let parent = hold.offsetParent
    document.getElementById('toc').appendChild(contents.list())
    document.addEventListener('scroll', (e) => {
        if (parent.offsetTop - document.documentElement.scrollTop < 0) {
            if (!hold.classList.contains('fixed-zero-top')) {
                hold.classList.add('fixed-zero-top')
            }
        } else {
            if (hold.classList.contains('fixed-zero-top')) {
                hold.classList.remove('fixed-zero-top')
            }
        }
    })
})