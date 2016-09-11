using System;
using System.Diagnostics;
using System.Collections.Generic;
using System.Threading;
namespace csharpclarity
{
    class Vector
    {
        public float x, y, z;

        public Vector(float x, float y, float z)
        {
            this.x = x;
            this.y = y;
            this.z = z;
        }

        public static Vector operator *(Vector a, Vector b)
        {
            return new Vector(a.x * b.x, a.y * b.y, a.z * b.z);
        }
        public static Vector operator +(Vector a, Vector b)
        {
            return new Vector(a.x + b.x, a.y + b.y, a.z + b.z);
        }

        public static Vector operator -(Vector a, Vector b)
        {
            return new Vector(a.x - b.x, a.y - b.y, a.z - b.z);
        }

        public static float getDistance(Vector a, Vector b)
        {
            Vector s = a - b;
            return (float)Math.Sqrt(s.x * s.x + s.y * s.y + s.z * s.z);
        }
    }


    class Block
    {
        public Vector location; //x,y,z within the chunk        
        public string name;
        public int durability;
        public int textureid;
        public bool breakable;
        public bool visible;
        public int type;

        public Block(Vector location, string name, int durability, int textureid, bool breakable, bool visible, int type)
        {
            this.location = location;
            this.name = name;
            this.durability = durability;
            this.textureid = textureid;
            this.breakable = breakable;
            this.visible = visible;
            this.type = type;
        }
    }

    class Entity
    {        
        public Vector location; //x,y,z within the chunk
        public string name;
        public int health;
        public Vector speed;
        public enum Type { Zombie, Chicken, Exploder, TallCreepyThing };

        public Entity(Vector location, Type type)
        {
            this.location = location;

            switch (type)
            {
                case Type.Zombie:
                    name = "Zombie";
                    health = 50;
                    speed = new Vector(0.5f, 0.0f, 0.5f); //slow, can't fly
                    break;
                case Type.Chicken:
                    name = "Chicken";
                    health = 25;
                    speed = new Vector(0.75f, 0.25f, 0.75f); //can fly a bit
                    break;
                case Type.Exploder:
                    name = "Exploder";
                    health = 75;
                    speed = new Vector(0.75f, 0.0f, 0.75f);
                    break;
                case Type.TallCreepyThing:
                    name = "Tall Creepy Thing";
                    health = 500;
                    speed = new Vector(1.0f, 1.0f, 1.0f); //does what he wants
                    break;
            }

        }

        public void updatePosition()
        {
            // Complex movement AI
            Vector rndUnitIshVector = new Vector(1,1,1);
            Vector movementVector = rndUnitIshVector * speed;
            location = movementVector + location;
        }

    }


    class Chunk
    {
        private static readonly int NUM_BLOCKS = 65536; //just like minecraft!
        private static readonly int NUM_ENTITIES = 1000;

        List<Block> blocks;
        List<Entity> entities;
        public Vector location; //x,y,z within world

        public Chunk(Vector location)
        {
            this.location = location;
            //Preallocate the growable List because we are clever!
            blocks = new List<Block>(NUM_BLOCKS);
            for (int i = 0; i < NUM_BLOCKS; i++)
            {
                Block b = new Block(new Vector(i, i, i), "Block:" + i, 100, 1, true, true, 1);
                blocks.Add(b);
            }

            entities = new List<Entity>(NUM_ENTITIES);
            for (int i = 0; i < NUM_ENTITIES / 4; i++)
            {
                // Fancy proc gen initial position equation
                entities.Add(new Entity(new Vector(i, i, i), Entity.Type.Chicken));
                entities.Add(new Entity(new Vector(i+2, i, i), Entity.Type.Zombie));
                entities.Add(new Entity(new Vector(i+3, i, i), Entity.Type.Exploder));
                entities.Add(new Entity(new Vector(i+4, i, i), Entity.Type.TallCreepyThing));

            }
        }

        public void processEntities()
        {
            foreach (var entity in entities)
            {
                entity.updatePosition();
            }
        }
    }

    class Game
    {
        

        private static readonly int CHUNK_COUNT = 100;
        public static List<Chunk> chunks;
        public static Vector playerLocation = new Vector(0, 0, 0);
        public static int chunkCounter = 0;


        static void loadWorld()
        {
            chunks = new List<Chunk>(CHUNK_COUNT);            
            for (int i = 0; i < CHUNK_COUNT; i++)
            {                
                chunks.Add(new Chunk(new Vector(chunkCounter, 0.0f, 0.0f)));
                chunkCounter++;
            }
        }


        static void updateChunks()
        {
            List<Chunk> toRemove = new List<Chunk>(2);
            foreach (var chunk in chunks)
            {
                chunk.processEntities();
                float chunkDistance = Vector.getDistance(chunk.location, playerLocation);
                if (chunkDistance > CHUNK_COUNT)
                {
                    toRemove.Add(chunk);

                }                
            }

            foreach (var chunk in toRemove)
            {
                chunks.Remove(chunk);
                chunks.Add(new Chunk(new Vector(chunkCounter, 0.0f, 0.0f)));
                chunkCounter++;
            }
        }
                

        static void Main(string[] args)
        {
            // Praise be to the RNG
            
            Stopwatch s = new Stopwatch();
            
            Console.Write("Loading World...");
            s.Start();
            loadWorld();
            s.Stop();
            Console.WriteLine("FINISHED!");
            Console.WriteLine("Load Time:" + s.ElapsedMilliseconds);
            //Game Loop, you can never leave
            while (true)
            {
                //check for dead entities
                s.Restart();

                // mocking polling of the VR controller
                Vector playerMovement = new Vector(0.1f, 0.0f, 0.0f);
                playerLocation += playerMovement;
                updateChunks();

                s.Stop();
                double time = ((double)s.ElapsedTicks / (double)Stopwatch.Frequency) * 1000.0;

                Console.WriteLine(time);
                //Lock it at 60FPS
                if (time < 16.0)
                {

                    Thread.Sleep((int)(16.0 - time));
                }
            }
        }
    }
}
