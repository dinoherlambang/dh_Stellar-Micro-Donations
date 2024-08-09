import React, { useState } from 'react';
import { StellarService } from '../StellarService';

interface CreateProjectFormProps {
  stellarService: StellarService;
}

const CreateProjectForm: React.FC<CreateProjectFormProps> = ({ stellarService }) => {
  const [name, setName] = useState('');
  const [goal, setGoal] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await stellarService.createProject(name, Number(goal));
      alert('Project created successfully!');
      setName('');
      setGoal('');
    } catch (error) {
      console.error('Error creating project:', error);
      alert('Error creating project. Please try again.');
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <h2>Create New Project</h2>
      <div>
        <label htmlFor="name">Project Name:</label>
        <input
          type="text"
          id="name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          required
        />
      </div>
      <div>
        <label htmlFor="goal">Fundraising Goal:</label>
        <input
          type="number"
          id="goal"
          value={goal}
          onChange={(e) => setGoal(e.target.value)}
          required
        />
      </div>
      <button type="submit">Create Project</button>
    </form>
  );
};

export default CreateProjectForm;