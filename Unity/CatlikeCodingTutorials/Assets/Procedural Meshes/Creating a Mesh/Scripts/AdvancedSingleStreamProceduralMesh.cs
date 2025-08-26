using System.Runtime.InteropServices;
using Unity.Collections;
using Unity.Mathematics;
using UnityEngine;
using UnityEngine.Rendering;

[RequireComponent(typeof(MeshFilter), typeof(MeshRenderer))]
public class AdvancedSingleStreamProceduralMesh : MonoBehaviour
{
    [StructLayout(LayoutKind.Sequential)]
    struct Vertex
    {
        public float3 position, normal;
        public half4 tangent;
        public half2 texCoord0;
    }

    private void OnEnable()
    {
        var vertexAttributeCount = 4;
        var vertexCount = 4;
        var triangleIndexCount = 6;

        var meshDataArray = Mesh.AllocateWritableMeshData(1);
        var meshData = meshDataArray[0];

        var vertexAttributes = new NativeArray<VertexAttributeDescriptor>(vertexAttributeCount, Allocator.Temp, NativeArrayOptions.UninitializedMemory);
        vertexAttributes[0] = new VertexAttributeDescriptor(dimension: 3);
        vertexAttributes[1] = new VertexAttributeDescriptor(VertexAttribute.Normal, dimension: 3);
        vertexAttributes[2] = new VertexAttributeDescriptor(VertexAttribute.Tangent, VertexAttributeFormat.Float16, dimension: 4);
        vertexAttributes[3] = new VertexAttributeDescriptor(VertexAttribute.TexCoord0, VertexAttributeFormat.Float16, dimension: 2);
        
        meshData.SetVertexBufferParams(vertexCount, vertexAttributes);

        vertexAttributes.Dispose();

        var vertices = meshData.GetVertexData<Vertex>();

        half h1 = math.half(1), h0 = math.half(0);

        var vertex = new Vertex { 
            normal = math.back(),
            tangent = math.half4(h1, h0, h0, math.half(-1))
        };

        vertex.position = 0;
        vertex.texCoord0 = h0;
        vertices[0] = vertex;

        vertex.position = math.right();
        vertex.texCoord0 = math.half2(h1, h0);
        vertices[1] = vertex;

        vertex.position = math.up();
        vertex.texCoord0 = math.half2(h0, h1);
        vertices[2] = vertex;

        vertex.position = math.float3(1, 1, 0);
        vertex.texCoord0 = h1;
        vertices[3] = vertex;

        meshData.SetIndexBufferParams(triangleIndexCount, IndexFormat.UInt16);

        var triangleIndices = meshData.GetIndexData<ushort>();
        triangleIndices[0] = 0;
        triangleIndices[1] = 2;
        triangleIndices[2] = 1;
        triangleIndices[3] = 1;
        triangleIndices[4] = 2;
        triangleIndices[5] = 3;

        var bounds = new Bounds(new Vector3(0.5f, 0.5f), new Vector3(1, 1));

        meshData.subMeshCount = 1;
        meshData.SetSubMesh(0, new SubMeshDescriptor(0, triangleIndexCount) {
            bounds = bounds,
            vertexCount = vertexCount
        }, MeshUpdateFlags.DontRecalculateBounds);

        var mesh = new Mesh
        {
            name = "Procedural mesh",
            bounds = bounds
        };

        Mesh.ApplyAndDisposeWritableMeshData(meshDataArray, mesh);

        GetComponent<MeshFilter>().mesh = mesh;
    }
}