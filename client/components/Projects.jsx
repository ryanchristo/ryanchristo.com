import React, { PropTypes } from 'react'
import styles from './Projects.scss'

const Projects = ({
  nextProject,
  previousProject,
  projects,
  selectedId,
  setProject,
  showProjects,
}) => (
  <div className={styles.projects}>
    <div className={showProjects ? styles.index : styles.indexHidden}>
      {projects.map(project => (
        <button
          key={project.id}
          className={selectedId === project.id ? styles.activeIndexItem : styles.indexItem}
          onClick={() => setProject(project)}
        >
          {project.title}
        </button>
      ))}
    </div>
    <div className={showProjects ? styles.slideshow : styles.slideshowHidden}>
      <div className={styles.slideshowContent}>
        {projects.map(project => (
          <button
            key={project.id}
            className={selectedId === project.id ? styles.activeImage : styles.image}
            onClick={() => setProject(project)}
          >
            <img
              alt={project.title}
              key={project.id}
              src={project.image}
            />
          </button>
        ))}
      </div>
    </div>
    <div id={`${projects[0].type}-project`} className={showProjects ? styles.project : styles.projectHidden}>
      <div className={showProjects ? styles.leftArrowContainer : styles.leftArrowContainerHidden}>
        <button className={styles.leftArrow} onClick={previousProject}>
          {'<'}
        </button>
      </div>
      {projects.map(project => (
        <div key={project.id}>
          {selectedId === project.id &&
            <div className={styles.text}>
              <h3 className={styles.title}>{project.title}</h3>
              <p className={styles.summary}>{project.summary}</p>
              <p className={styles.summary}>{project.tags}</p>
              <div className={styles.links}>
                <a
                  className={styles.link}
                  href={project.link.production}
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  View Slideshow
                </a>
                {project.link.repository &&
                  <a
                    className={styles.link}
                    href={project.link.repository}
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    View GitHub
                  </a>
                }
              </div>
            </div>
          }
        </div>
      ))}
      <div className={showProjects ? styles.rightArrowContainer : styles.rightArrowContainerHidden}>
        <button className={styles.rightArrow} onClick={nextProject}>
          {'>'}
        </button>
      </div>
    </div>
  </div>
)

Projects.propTypes = {
  nextProject: PropTypes.func.isRequired,
  previousProject: PropTypes.func.isRequired,
  projects: PropTypes.arrayOf(
    PropTypes.shape({
      id: PropTypes.string.isRequired,
      image: PropTypes.string.isRequired,
      link: PropTypes.shape({
        production: PropTypes.string.isRquired,
        repository: PropTypes.string,
      }).isRequired,
      summary: PropTypes.string.isReuquired,
      title: PropTypes.string.isRequired,
    }).isRequired,
  ).isRequired,
  selectedId: PropTypes.string.isRequired,
  setProject: PropTypes.func.isRequired,
  showProjects: PropTypes.bool.isRequired,
}

export default Projects