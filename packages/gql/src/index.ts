import { graphql, buildSchema } from 'graphql';
import fs from 'fs/promises';
import { homedir } from 'os';
import { join, dirname } from 'path';

const schema = buildSchema(`
  type Query {
    config: String
  }
`);

const rootValue = {
  config: async () => {
    const configPath = join(
      homedir(),
      '.config',
      'ciska',
      process.env.NODE_ENV,
      'config.json',
    );

    const accessResult = await fs.access(configPath).catch((err: Error) => err);
    if (accessResult instanceof Error) {
      await fs.writeFile(configPath, '{}');
    }

    await fs.mkdir(dirname(configPath), { recursive: true });

    return JSON.parse(await fs.readFile(configPath, 'utf8'));
  },
};

export async function search() {
  return await graphql({
    schema,
    source: '{ projects }',
    rootValue,
  });
}

search().then(console.log);
