using UnityEngine;

namespace ProceduralMeshes
{
    public interface IMeshGenerator
    {
        int VertexCount { get; }
        int IndexCount { get; }
        Bounds Bounds { get; }

        int Resolution { get; set; }

        int JobLength { get; }

        void Execute<S>(int i, S streams) where S : struct, IMeshStreams;
    }
}