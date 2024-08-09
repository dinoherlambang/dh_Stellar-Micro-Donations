import React from 'react';
import { StellarService } from './StellarService';
import ProjectList from './components/ProjectList';
import DonationForm from './components/DonationForm';
import CreateProjectForm from './components/CreateProjectForm';

const App: React.FC = () => {
  const stellarService = new StellarService(
    process.env.REACT_APP_HORIZON_URL || '',
    process.env.REACT_APP_CONTRACT_ID || '',
    process.env.REACT_APP_ADMIN_SECRET_KEY || ''
  );

  return (
    <div className="App">
      <h1>Micro-Donations Platform</h1>
      <ProjectList stellarService={stellarService} />
      <DonationForm stellarService={stellarService} />
      <CreateProjectForm stellarService={stellarService} />
    </div>
  );
};

export default App;