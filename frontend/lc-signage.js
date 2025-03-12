/*
File::lc-signage.js
processJSON() processes and displays the JSON file.
	The JSON file contains all public calendar events for the specific room.
	JSON file is parsed for content that is for today, and has not ended yet.
getTime() Adds and updates time and date displayed on the screen.

Created by LG 2024::Madison County Public Library
Email::lucas@madisonlibrary.org
Website::https://madisonlibrary.org/
*/
function processJSON(feed){
    var data = feed.responseJSON;
    if(feed.status!=200 && feed.status!=304)//If the script fails to pull content from the server, it does not update the screen.
        return;
	var html = "<h1>"+room+"</h1>";// Start of HTML, add room identifier to page.
	$('#main').html(html);
	if(data.length<1){
		html = "<center><h3><br>This room is available for reservation</h3></center></div>";//Display this message if there are no events to display.
		$('#main').append(html);	
	}
	else {
		for(i = 0; i < data.length; i++){
		    if(data[i].moderation_state=="published" || data[i].moderation_state=="approved" || data[i].moderation_state=="cancelled"){//Only show APPROVED events
		        if(data[i].public==true)//adds public/private denotion to events.
		            data[i].title = "<img src='[PUBLIC_TRUE]' style='height:6vmin;' /> "+data[i].title;
		        else
		            data[i].title = "<img src='[PUBLIC_FALSE]' style='height:6vmin;' /> "+data[i].title;
			    html = "<div id=\"mcpl_"+data[i].id+"\" class=\"left\">";//Start container with unique id.
			    var cancel_style = "";
			    if(data[i].moderation_state=="cancelled")//If an event state is set to cancelled, strikethrough text.
        			cancel_style = "style='text-decoration:line-through;'";
			    if(i==0){
				    html += "<h2 "+cancel_style+">"+data[i].title+" <span class=\"roomAppend\">"+roomAppend+"</span></h2>";//First event gets larger text.
			    } else {
				    html += "<h3 "+cancel_style+">"+data[i].title+" <span class=\"roomAppend\">"+roomAppend+"</span></h3>";//All other events get smaller text.
			    }
			    if(data[i].moderation_state=="cancelled")//If an event state is set to cancelled, set the time to 'cancelled'.
			        html += "<h4>Cancelled</h4>"
			    else
			        html += "<h4>"+data[i].start_time+" - "+data[i].end_time+"</h4></div>";//Add start and end time.
			    $('#main').append(html);
			}
		}
	}
	getTime();
	/*This reloads page content every 5 minutes (300,000 miliseconds)*/
	window.setTimeout(function(){$.ajax({url: feedURL, complete: function(feed){processJSON(feed);}});}, 300000);
	/**************************************************************/
}
function getTime(){
	var months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
	var d = new Date();
	var h = d.getHours();
	var m = d.getMinutes();
	if(h>11)a="PM";else a="AM";
	if(h>12)h-=12;
	if(m<10)m="0"+m;
	$('#time').html("<h5>"+months[d.getMonth()]+" "+d.getDate()+", "+d.getFullYear()+"<span style=\"float:right;\">"+h+":"+m+" "+a+"</span></h5>");
	window.setTimeout(function(){getTime();}, 10000);
}
/*AJAX call, using jQuery, to pull JSON feed*/
$(document).ready(function(){$.ajax({url: feedURL, complete: function(feed){processJSON(feed);}});});
/********************************************************************/
