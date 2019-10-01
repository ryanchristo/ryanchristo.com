import React, { Component } from 'react'

import About from './About'
import Connect from './Connect'
import Header from './Header'
import Home from './Home'
import Loader from './Loader'
import Media from './Media'
import Software from './Software'

import background1 from '../assets/img/background-1.jpg'
import background2 from '../assets/img/background-2.jpg'
import profile from '../assets/img/profile.jpg'

import styles from './Landing.module.scss'

class Landing extends Component {
  constructor(props) {
    super(props)
    this.state = {
      currentView: 'home',
      loaded: 0,
      loading: true,
      showConnect: false,
      showHeader: false,
      showHome: true,
    }
    this.handleScrollEvent = this.handleScrollEvent.bind(this)
  }

  componentDidMount() {
    window.addEventListener('scroll', this.handleScrollEvent)
    this.loadImages()
  }

  componentWillUnmount() {
    window.removeEventListener('scroll', this.handleScrollEvent)
  }

  handleScrollEvent() {
    const about = document.getElementById('about').getBoundingClientRect()
    const connect = document.getElementById('connect').getBoundingClientRect()
    const home = document.getElementById('home').getBoundingClientRect()
    const media = document.getElementById('media').getBoundingClientRect()
    const software = document.getElementById('software').getBoundingClientRect()

    let { currentView, showConnect, showHeader, showHome } = this.state

    if (window.scrollY > 500) {
      showHeader = true
    } else {
      showHeader = false
    }
    if (home.top > -500) {
      currentView = 'home'
      showHome = true
    } else {
      showHome = false
    }
    if (about.top < 800 && about.bottom > 200) {
      currentView = 'about'
    }
    if (software.top < 800 && software.bottom > 200) {
      currentView = 'software'
    }
    if (media.top < 800 && media.bottom > 200) {
      currentView = 'media'
    }
    if (connect.top < 500) {
      currentView = 'connect'
      showConnect = true
    } else {
      showConnect = false
    }

    const {
      currentView: stateCurrentView,
      showConnect: stateShowConnect,
      showHeader: stateShowHeader,
      showHome: stateShowHome,
    } = this.state

    if (
      currentView !== stateCurrentView ||
      showConnect !== stateShowConnect ||
      showHeader !== stateShowHeader ||
      showHome !== stateShowHome
    ) {
      this.setState({
        currentView,
        showConnect,
        showHeader,
        showHome,
      })
    }
  }

  loadImages() {
    // eslint-disable-next-line no-undef
    const image1 = new Image()
    // eslint-disable-next-line no-undef
    const image2 = new Image()
    // eslint-disable-next-line no-undef
    const image3 = new Image()

    image1.src = background1
    image2.src = background2
    image3.src = profile

    let { loaded } = this.state

    const updateLoading = () => {
      loaded += 1
      this.setState({ loaded })
      if (loaded === 3) {
        this.setState({ loading: false })
      }
    }

    image1.onload = () => {
      updateLoading()
    }
    image2.onload = () => {
      updateLoading()
    }
    image3.onload = () => {
      updateLoading()
    }
  }

  render() {
    const {
      currentView,
      loading,
      showConnect,
      showHeader,
      showHome,
    } = this.state

    if (loading) return <Loader />

    return (
      <div className={styles.container}>
        <Header currentView={currentView} showHeader={showHeader} />
        <Home showHome={showHome} />
        <About />
        <Software />
        <Media />
        <Connect showConnect={showConnect} />
      </div>
    )
  }
}

export default Landing