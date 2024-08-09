import React, { useState } from 'react';
import { StellarService } from '../StellarService';

interface DonationFormProps {
  stellarService: StellarService;
}

const DonationForm: React.FC<DonationFormProps> = ({ stellarService }) => {
  const [project, setProject] = useState('');
  const [amount, setAmount] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await stellarService.donate(project, Number(amount));
      alert('Donation successful!');
      setProject('');
      setAmount('');
    } catch (error) {
      console.error('Error making donation:', error);
      alert('Error making donation. Please try again.');
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <h2>Make a Donation</h2>
      <div>
        <label htmlFor="project">Project:</label>
        <input
          type="text"
          id="project"
          value={project}
          onChange={(e) => setProject(e.target.value)}
          required
        />
      </div>
      <div>
        <label htmlFor="amount">Amount:</label>
        <input
          type="number"
          id="amount"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          required
        />
      </div>
      <button type="submit">Donate</button>
    </form>
  );
};

export default DonationForm;