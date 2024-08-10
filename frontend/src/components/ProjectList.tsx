import React, { useState, useEffect } from 'react';
import { StellarService } from '../StellarService';

interface Project {
  name: string;
  currentAmount: number;
  goal: number;
}

interface ProjectListProps {
  stellarService: StellarService;
}

const ProjectList: React.FC<ProjectListProps> = ({ stellarService }) => {
  const [projects, setProjects] = useState<Project[]>([]);

  useEffect(() => {
    const fetchProjects = async () => {
      try {
        const projectNames = await stellarService.getProjects();
        const projectDetails = await Promise.all(
          projectNames.map(async (name: string) => {
            const status = await stellarService.getProjectStatus(name);
            return { name, currentAmount: status[0], goal: status[1] };
          })
        );
        setProjects(projectDetails);
      } catch (error) {
        console.error('Error fetching projects:', error);
      }
    };

    fetchProjects();
  }, [stellarService]);

  return (
    <div>
      <h2>Donation Projects</h2>
      <ul>
        {projects.map((project) => (
          <li key={project.name}>
            {project.name}: {project.currentAmount} / {project.goal}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default ProjectList;