using System;
using UnityEngine;
using Unity.Mathematics;

public class Clock : MonoBehaviour
{
    [SerializeField]
    Transform hoursPivot, minutesPivot, secondsPivot;

    void Update()
    {
        var now = DateTime.Now.TimeOfDay;
        var hour = now.TotalHours % 12;
        var minute = now.TotalMinutes % 60;
        var second = now.TotalSeconds % 60;

        hoursPivot.localRotation = Quaternion.Euler(0, 0, -math.remap(0f, 12f, 0f, 360f, (float)hour));
        minutesPivot.localRotation = Quaternion.Euler(0, 0, -math.remap(0f, 60f, 0f, 360f, (float)minute));
        secondsPivot.localRotation = Quaternion.Euler(0, 0, -math.remap(0f, 60f, 0f, 360f, (float)second));
    }
}