import React, { PropTypes } from 'react'
import NavBarList from './NavBarList'
import styles from './NavBar.scss'

const NavBar = ({ current, scrollTo, showNavBar }) => (
  <nav id="navbar" className={showNavBar ? styles.container : styles.hidden}>
    <h1 className={styles.title} onClick={() => scrollTo('Home')}>
      rc | ryan christoffersen
    </h1>
    <NavBarList current={current} scrollTo={scrollTo} />
  </nav>
)

NavBar.propTypes = {
  current: PropTypes.string.isRequired,
  scrollTo: PropTypes.func.isRequired,
  scrollToTop: PropTypes.func.isRequired,
  showNavBar: PropTypes.bool.isRequired
}

export default NavBar
