package com.example.hzimmerman.sesame;

import android.net.ConnectivityManager;
import android.net.Network;
import android.os.Bundle;
import android.os.Handler;
import android.os.Message;
import android.support.wearable.activity.WearableActivity;
import android.widget.TextView;

import com.android.volley.Request;
import com.android.volley.RequestQueue;
import com.android.volley.Response;
import com.android.volley.VolleyError;
import com.android.volley.toolbox.StringRequest;
import com.android.volley.toolbox.Volley;


public class ToggleServoActivity extends WearableActivity {

    // Textview to show what is going on
    private TextView mTextView;

    // When the Activity is created, send a POST to the raspberry pi.
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        mTextView = (TextView) findViewById(R.id.text); // set up the text view

        // Instantiate the RequestQueue.
        RequestQueue queue = Volley.newRequestQueue(this);
        // armbar-abode.mooo.com is the domain name of my domicile.
        // The request is port forwarded to the raspberry pi.
        String url ="http://armbar-abode.mooo.com:8001/";

        // Request a response from the provided URL.
        StringRequest stringRequest = new StringRequest(Request.Method.POST, url,
                new Response.Listener<String>() {
                    @Override
                    public void onResponse(String response) {
                        // Indicate with the text on screen that the lock was toggled
                        mTextView.setText("Toggled the lock.");
                    }
                }, new Response.ErrorListener() {
            @Override
            public void onErrorResponse(VolleyError error) {
                // Indicate that the watch failed to tell the server to toggle
                mTextView.setText("Failed to send message" + error);
            }
        });
        // Add the request to the RequestQueue.
        queue.add(stringRequest);

    }

}
