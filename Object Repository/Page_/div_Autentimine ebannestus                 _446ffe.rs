<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Autentimine ebannestus                 _446ffe</name>
   <tag></tag>
   <elementGuidId>a8bb7147-6f12-46cb-913e-0b3f53aa63db</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.auth-container</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>a1a2143b-0243-428b-9906-d3ba5c180897</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>auth-container</value>
      <webElementGuid>1e5e51f8-09e1-4fd8-88df-2114adeb1de8</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    

    

    
        
            Autentimine ebaõnnestus
        
        
    

    
    
    
    

    
        Logi sisse
    




    async function submitLoginForm() {
        let response;
        if (!this.email || !this.password) {
            this.errorMessage = 'Please fill in all fields.';
            return;
        }

        try {
             response = await post('/sessions', {email: this.email, password: this.password});
        } catch (e) {
            this.errorMessage = e.message;
            return false;
        }

        // Save the token to local storage
        if (response.sessionId) {
            // add the session ID into cookies
            document.cookie = `sessionId=${response.sessionId}; path=/; max-age=86400`;
            window.location.href = window.location.href;
        } else {
            showError(new DetailedError( 500, 'Error', 'The server returned a response without a session ID.'));
        }
        return false;
    }
    
        
        
            Error
        
        
            
                
                    
                
                Unknown message
            
        
    </value>
      <webElementGuid>0592893a-e870-4e06-85c4-a8664e131833</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]/div[@class=&quot;auth-container&quot;]</value>
      <webElementGuid>5f68da38-5ddf-4e55-a3c5-ca1ec7ebdc0c</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div</value>
      <webElementGuid>9c918320-3b0f-4270-bbe9-44da9f931750</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
    

    

    
        
            Autentimine ebaõnnestus
        
        
    

    
    
    
    

    
        Logi sisse
    




    async function submitLoginForm() {
        let response;
        if (!this.email || !this.password) {
            this.errorMessage = &quot; , &quot;'&quot; , &quot;Please fill in all fields.&quot; , &quot;'&quot; , &quot;;
            return;
        }

        try {
             response = await post(&quot; , &quot;'&quot; , &quot;/sessions&quot; , &quot;'&quot; , &quot;, {email: this.email, password: this.password});
        } catch (e) {
            this.errorMessage = e.message;
            return false;
        }

        // Save the token to local storage
        if (response.sessionId) {
            // add the session ID into cookies
            document.cookie = `sessionId=${response.sessionId}; path=/; max-age=86400`;
            window.location.href = window.location.href;
        } else {
            showError(new DetailedError( 500, &quot; , &quot;'&quot; , &quot;Error&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;The server returned a response without a session ID.&quot; , &quot;'&quot; , &quot;));
        }
        return false;
    }
    
        
        
            Error
        
        
            
                
                    
                
                Unknown message
            
        
    &quot;) or . = concat(&quot;
    

    

    
        
            Autentimine ebaõnnestus
        
        
    

    
    
    
    

    
        Logi sisse
    




    async function submitLoginForm() {
        let response;
        if (!this.email || !this.password) {
            this.errorMessage = &quot; , &quot;'&quot; , &quot;Please fill in all fields.&quot; , &quot;'&quot; , &quot;;
            return;
        }

        try {
             response = await post(&quot; , &quot;'&quot; , &quot;/sessions&quot; , &quot;'&quot; , &quot;, {email: this.email, password: this.password});
        } catch (e) {
            this.errorMessage = e.message;
            return false;
        }

        // Save the token to local storage
        if (response.sessionId) {
            // add the session ID into cookies
            document.cookie = `sessionId=${response.sessionId}; path=/; max-age=86400`;
            window.location.href = window.location.href;
        } else {
            showError(new DetailedError( 500, &quot; , &quot;'&quot; , &quot;Error&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;The server returned a response without a session ID.&quot; , &quot;'&quot; , &quot;));
        }
        return false;
    }
    
        
        
            Error
        
        
            
                
                    
                
                Unknown message
            
        
    &quot;))]</value>
      <webElementGuid>eab90ebe-078f-41b2-9f1c-1446639b2775</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
